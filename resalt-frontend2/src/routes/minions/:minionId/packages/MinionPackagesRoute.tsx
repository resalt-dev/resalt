import { Card, CardHeader, SkeletonItem } from '@fluentui/react-components';
import { useEffect, useState } from 'react';
import { ObjectView } from 'react-object-view';
import { useParams } from 'react-router-dom';
import { CopyButton } from '../../../../components/CopyButton';
import { getMinionById } from '../../../../lib/api';
import { ToastController } from '../../../../lib/toast';
import { jsonPalette, sortedObject } from '../../../../lib/ui';
import MinionHeader from '../MinionHeader';

export default function MinionPackagesRoute(props: { toastController: ToastController }) {
	const { toastController } = props;
	const [loadingError, setLoadingError] = useState<Error | undefined>(undefined);
	const [packages, setPackages] = useState<any | null | undefined>(undefined);
	const minionId = useParams().minionId!;

	useEffect(() => {
		// Fetch minion
		const abort = new AbortController();
		getMinionById(minionId, abort.signal)
			.then((minion) => {
				if (minion.pkgs === null) {
					setPackages(null);
				} else {
					setPackages(JSON.parse(minion.pkgs));
				}
			})
			.catch((err: Error) => {
				setLoadingError(err);
				toastController.error('Error loading minion', err);
			});
		return () => {
			abort.abort();
		};
	}, [minionId, toastController]);

	return (
		<>
			<MinionHeader tab="packages" minionId={minionId!} error={loadingError} />
			<div className="fl-grid">
				<div className="fl-span-12">
					<Card style={{ height: '100%' }}>
						<CardHeader
							header={
								<>
									Packages
									<CopyButton
										name="packages"
										value={JSON.stringify(packages, null, '\t')}
										toastController={toastController}
									/>
								</>
							}
						/>
						{packages === undefined ? (
							<SkeletonItem />
						) : packages === null ? (
							<i>Unknown</i>
						) : (
							<ObjectView
								data={sortedObject(packages)}
								options={{
									hideDataTypes: true,
								}}
								styles={{}}
								palette={jsonPalette}
							/>
						)}
					</Card>
				</div>
			</div>
		</>
	);
}
