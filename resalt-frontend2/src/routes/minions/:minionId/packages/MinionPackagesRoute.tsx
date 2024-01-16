import { useParams } from 'react-router-dom';
import MinionHeader from '../MinionHeader';

export default function MinionPackagesRoute() {
	const { minionId } = useParams();

	return (
		<>
			<MinionHeader tab="packages" minionId={minionId!} />
		</>
	);
}
