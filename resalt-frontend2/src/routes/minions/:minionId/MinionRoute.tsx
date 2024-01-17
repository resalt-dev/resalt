import {
	Card,
	CardHeader,
	Table,
	TableBody,
	TableCell,
	TableHeader,
	TableHeaderCell,
	TableRow,
} from '@fluentui/react-components';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { getMinionById } from '../../../lib/api';
import { ToastController } from '../../../lib/toast';
import { multilineText, useGlobalStyles } from '../../../lib/ui';
import Minion from '../../../models/Minion';
import MinionHeader from './MinionHeader';

type Value = string | number | boolean | null | undefined;
type KeyValue = { k: string; v: Value; v2?: Value };
function InfoTable(props: { header?: KeyValue; items: KeyValue[] }) {
	return (
		<Table>
			{props.header && (
				<TableHeader>
					<TableRow>
						<TableHeaderCell key={props.header.k}>
							<strong>{props.header.k}</strong>
						</TableHeaderCell>
						<TableHeaderCell key={props.header.v?.toString()}>
							<strong>{props.header.v}</strong>
						</TableHeaderCell>
						{props.header.v2 && (
							<TableHeaderCell key={props.header.v2.toString()}>
								<strong>{props.header.v2}</strong>
							</TableHeaderCell>
						)}
					</TableRow>
				</TableHeader>
			)}
			<TableBody>
				{props.items.map((item) => (
					<TableRow key={item.k}>
						<TableCell>
							<strong>{item.k}</strong>
						</TableCell>
						<TableCell>
							{item.v && item.v.toString().length > 0 ? (
								multilineText(item.v)
							) : (
								<i>Unknown</i>
							)}
						</TableCell>
					</TableRow>
				))}
			</TableBody>
		</Table>
	);
}

function InfoCard(props: { title: string; header?: KeyValue; items: KeyValue[] }) {
	const globalStyles = useGlobalStyles();
	return (
		<Card>
			<CardHeader
				header={<span className={globalStyles.cardHeaderTitle}>{props.title}</span>}
			/>
			<InfoTable header={props.header} items={props.items} />
		</Card>
	);
}

export default function MinionRoute(props: { toastController: ToastController }) {
	const { toastController } = props;
	const [minion, setMinion] = useState<Minion | null>(null);
	const [grains, setGrains] = useState<any>({});
	const minionId = useParams().minionId!;

	useEffect(() => {
		// Fetch minion
		const abort = new AbortController();
		getMinionById(minionId, abort.signal)
			.then((minion) => {
				setMinion(minion);
				setGrains(JSON.parse(minion?.grains ?? '{}'));
			})
			.catch((err) => {
				toastController.error('Error loading Minion', err);
			});
		return () => {
			abort.abort();
		};
	}, [minionId, toastController]);

	return (
		<>
			<MinionHeader tab="" minionId={minionId!} />
			<div className="fl-grid">
				<div className="fl-span-3">
					<InfoCard
						title="Common"
						items={[
							{ k: 'ID', v: minion?.id },
							{ k: 'F.Q.D.N', v: grains?.fqdn },
							{ k: 'OS', v: grains?.os },
							{ k: 'OS Version', v: grains?.osrelease },
							{ k: 'Serial Number', v: grains?.serialnumber },
						]}
					/>
				</div>
				<div className="fl-span-3">
					<InfoCard
						title="Hardware"
						items={[
							{ k: 'CPU', v: grains.cpu_model },
							{ k: 'Number of CPUs', v: grains.num_cpus },
							{ k: 'Memory', v: grains.mem_total },
							{ k: 'Swap', v: grains.swap_total },
							{ k: 'Virtual', v: grains.virtual },
						]}
					/>
				</div>
				<div className="fl-span-3">
					<InfoCard
						title="DNS"
						items={[
							{
								k: 'IPv4 DNS',
								v: grains?.dns?.ip4_nameservers?.join('\n'),
							},
							{
								k: 'IPv6 DNS',
								v: grains?.dns?.ip6_nameservers?.join('\n'),
							},
							{
								k: 'Search Domains',
								v: grains?.dns?.search?.join('\n'),
							},
						]}
					/>
				</div>
				<div className="fl-span-3">
					<InfoCard
						title="Timings"
						items={[
							{ k: 'Last seen', v: minion?.lastSeen },
							{ k: 'Conformity check', v: minion?.lastUpdatedConformity },
							{ k: 'Grains fetched', v: minion?.lastUpdatedGrains },
							{ k: 'Pillars fetched', v: minion?.lastUpdatedPillars },
							{ k: 'Packages fetched', v: minion?.lastUpdatedPkgs },
						]}
					/>
				</div>
				<div className="fl-span-12">
					<InfoCard
						title="Network"
						header={{ k: 'Interface', v: 'Address', v2: 'MAC' }}
						items={Object.entries(
							(grains.ip_interfaces ?? {}) as { [key: string]: string[] },
						).map(([k, v]) => {
							return { k: k, v: v.join('\n') };
						})}
					/>
				</div>
			</div>
		</>
	);
}
