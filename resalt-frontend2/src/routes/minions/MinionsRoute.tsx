import {
	Body1,
	Button,
	Caption1,
	Card,
	CardFooter,
	CardHeader,
	makeStyles,
	typographyStyles,
} from '@fluentui/react-components';
import { ArrowReplyRegular, MoreHorizontal20Regular, ShareRegular } from '@fluentui/react-icons';
import { useGlobalStyles } from '../../lib/ui';

const useStyles = makeStyles({
	presetList: {},
	presetItem: {},
	presetItemTitle: {
		...typographyStyles.body2,
	},
	presetItemCaption: {
		...typographyStyles.caption1,
	},
});

export default function MinionsRoute() {
	const globalStyles = useGlobalStyles();
	const styles = useStyles();

	const presets = ['ALL MINIONS', 'NO - Oslo', 'SE - Lund'];

	return (
		<div>
			<div className="fl-grid">
				<div className="fl-span-2">
					<div className={globalStyles.title}>Minions</div>
				</div>
			</div>
			<div className="fl-grid">
				<Card className="fl-span-3 p-0">
					{presets.map((preset) => (
						<Card
							onClick={() => console.log('clicked')}
							appearance="subtle"
							key={preset}
							className={styles.presetItem}
							orientation="horizontal"
						>
							<CardHeader
								header={<div className={styles.presetItemTitle}>App Name</div>}
								description={
									<Caption1 className={styles.presetItemCaption}>
										Developer
									</Caption1>
								}
								action={
									<Button
										appearance="transparent"
										icon={<MoreHorizontal20Regular />}
										aria-label="More options"
									/>
								}
							/>
						</Card>
					))}
				</Card>
				<Card className="fl-span-9">
					<CardHeader
						header={
							<Body1>
								<b>Elvia Atkins</b> mentioned you
							</Body1>
						}
						description={<Caption1>5h ago · About us - Overview</Caption1>}
					/>

					<p>Hi lorum ipsum dolor etat</p>

					<CardFooter>
						<Button icon={<ArrowReplyRegular fontSize={16} />}>Reply</Button>
						<Button icon={<ShareRegular fontSize={16} />}>Share</Button>
					</CardFooter>
				</Card>
			</div>
		</div>
	);
}
