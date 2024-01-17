import { Tab, TabList } from '@fluentui/react-components';
import { Link } from 'react-router-dom';
import { paths } from '../../../lib/paths';
import { useGlobalStyles } from '../../../lib/ui';

export type MinionTab = '' | 'grains' | 'conformity' | 'pillars' | 'packages';
export default function MinionHeader(props: { tab: MinionTab; minionId: string }) {
	const { tab, minionId } = props;
	const globalStyles = useGlobalStyles();

	return (
		<>
			<div className="fl-grid">
				<div className="fl-span-3">
					<div className={globalStyles.title}>
						<paths.minion.Icon />
						{minionId}
					</div>
				</div>
			</div>
			<div className="fl-grid">
				<div className="fl-span-12">
					<TabList defaultSelectedValue={tab}>
						<Link to={paths.minion.getPath({ minionId })}>
							<Tab value="">Minion</Tab>
						</Link>
						<Link to={paths.minion_grains.getPath({ minionId })}>
							<Tab value="grains">Grains</Tab>
						</Link>
						<Link to={paths.minion_conformity.getPath({ minionId })}>
							<Tab value="conformity">Conformity</Tab>
						</Link>
						<Link to={paths.minion_pillars.getPath({ minionId })}>
							<Tab value="pillars">Pillars</Tab>
						</Link>
						<Link to={paths.minion_packages.getPath({ minionId })}>
							<Tab value="packages">Packages</Tab>
						</Link>
					</TabList>
				</div>
				<div className="fl-span-12" />
			</div>
		</>
	);
}
