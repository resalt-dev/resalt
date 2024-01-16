import { useParams } from 'react-router-dom';
import MinionHeader from '../MinionHeader';

export default function MinionGrainsRoute() {
	const { minionId } = useParams();

	return (
		<>
			<MinionHeader tab="grains" minionId={minionId!} />
		</>
	);
}
