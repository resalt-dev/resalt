import { useGlobalStyles } from '../../lib/ui';

export default function UsersRoute() {
	const globalStyles = useGlobalStyles();
	return (
		<>
			<div className="fl-grid">
				<div className="fl-span-2">
					<div className={globalStyles.title}>Users</div>
				</div>
			</div>
			<div className="fl-grid">
				<div className="fl-span-12">
					<div className={globalStyles.title}>user List</div>
				</div>
			</div>
		</>
	);
}
