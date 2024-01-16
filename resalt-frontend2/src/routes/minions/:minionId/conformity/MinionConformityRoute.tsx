import { useParams } from 'react-router-dom';
import MinionHeader from '../MinionHeader';

export default function MinionConformityRoute() {
	const { minionId } = useParams();

	return (
		<>
			<MinionHeader tab="conformity" minionId={minionId!} />
		</>
	);
}
