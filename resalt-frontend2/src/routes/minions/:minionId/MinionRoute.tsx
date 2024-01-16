import { useParams } from 'react-router-dom';
import MinionHeader from './MinionHeader';

export default function MinionRoute() {
	const { minionId } = useParams();

	return (
		<>
			<MinionHeader tab="" minionId={minionId!} />
			<div className="fl-grid">
				<div className="fl-span-12">aaaaaaaaaaaaaaaaaaaaaaa</div>
			</div>
		</>
	);
}
