import { useParams } from 'react-router-dom';
import MinionHeader from '../MinionHeader';

export default function MinionPillarsRoute() {
	const { minionId } = useParams();

	return (
		<>
			<MinionHeader tab="pillars" minionId={minionId!} />
		</>
	);
}
