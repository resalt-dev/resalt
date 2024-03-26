import {
	Card,
	CardHeader,
	Spinner,
	Switch,
	makeStyles,
	shorthands,
	tokens,
	useId,
} from '@fluentui/react-components';
import { useEffect, useMemo, useState } from 'react';
import { useParams } from 'react-router-dom';
import { TerminalArea } from '../../../../components/TerminalArea';
import { runJob } from '../../../../lib/api';
import { ToastController } from '../../../../lib/toast';
import { RunClientType } from '../../../../models/RunClientType';
import RunCommand from '../../../../models/RunCommand';
import MinionHeader from '../MinionHeader';

export const useStyles = makeStyles({
	terminalArea: {
		...shorthands.padding(tokens.spacingHorizontalXL),
		height: '65vh',
		overflowY: 'auto',
	},
	terminalInput: {
		backgroundColor: 'black',
		color: 'white',
	},
	terminalSpinner: {
		width: 'fit-content',
		...shorthands.padding(tokens.spacingHorizontalXL, tokens.spacingVerticalXXXL),
	},
	textUsername: {
		fontWeight: 'bold',
		color: tokens.colorPalettePinkBorderActive,
	},
	textAt: {
		fontWeight: 'bold',
		color: tokens.colorPalettePlatinumBackground2,
	},
	textHost: {
		fontWeight: 'bold',
		color: tokens.colorPaletteLightGreenBorderActive,
	},
	textPath: {
		fontWeight: 'bold',
		color: tokens.colorPaletteBlueBorderActive,
	},
});

export default function MinionTerminalRoute(props: { toastController: ToastController }) {
	const { toastController } = props;
	const styles = useStyles();
	const [loading, setLoading] = useState<boolean>(false);
	const [advancedMode, setAdvancedMode] = useState(false);
	const [output, setOutput] = useState<string>('');
	const minionId = useParams().minionId!;

	let terminalAreaId = useId('terminalArea');
	let inputId = useId('terminalInput');
	const magicPS1Placeholder = useMemo(() => 'magicPS1Placeholder' + crypto.randomUUID(), []);

	function scrollBottom() {
		const terminalArea = document.getElementById(terminalAreaId);
		if (terminalArea) {
			terminalArea.scrollTop = terminalArea.scrollHeight;
		}
	}
	useEffect(() => {
		scrollBottom();
	}, [output]);

	useEffect(() => {
		// refocus input when loading switches to false
		if (!loading) {
			const input = document.getElementById(inputId);
			if (input) {
				input.focus();
			}
		}
	}, [loading]);

	function runCmd(command: string) {
		command = command?.trim();
		if (!command) {
			return;
		}
		if (command === 'clear') {
			setOutput((_prev) => '');
			return;
		}

		setOutput((prev) => prev + magicPS1Placeholder + command + '\n');

		const abort = new AbortController();
		setLoading(true);
		runJob(
			new RunCommand(
				RunClientType.LOCAL,
				'list',
				minionId,
				'cmd.run',
				[command],
				new Map<string, string>(),
				'',
			),
			abort.signal,
		)
			.then((result) => {
				// Fetch result[minionId] and display it
				let resultString = (result as Record<string, unknown>)[minionId] as string;
				if (typeof resultString === 'boolean') {
					resultString = resultString ? 'True' : 'False';
				}
				resultString = typeof resultString === 'string' ? resultString.trim() : '';
				if (resultString) {
					setOutput((prev) => prev + resultString + '\n');
				}
				setLoading(false);
			})
			.catch((err: Error) => {
				toastController.error('Error running command', err);
				setLoading(false);
				setOutput((prev) => prev + 'Error running command: ' + err.message + '\n');
			});
	}

	const PS1 = (
		<>
			<span className={styles.textUsername}>root</span>
			<span className={styles.textAt}>@</span>
			<span className={styles.textHost}>{minionId}</span>
			<span>:</span>
			<span className={styles.textPath}>/root</span>
			#&nbsp;
		</>
	);

	return (
		<>
			<MinionHeader tab="terminal" minionId={minionId!} />
			<div className="fl-grid">
				<div className="fl-span-12">
					<Card style={{ height: '100%' }}>
						<CardHeader
							header="Terminal"
							action={
								<>
									Advanced:{' '}
									<Switch
										checked={advancedMode}
										onChange={(_ev, data) => setAdvancedMode(data.checked)}
									/>
								</>
							}
						/>
						{advancedMode ? (
							<>
								<p>Advanced mode</p>
							</>
						) : (
							<TerminalArea id={terminalAreaId} className={styles.terminalArea}>
								{output.split('\n').map((line, i) => (
									<div key={i}>
										{line.startsWith(magicPS1Placeholder) ? (
											<span>
												{PS1}
												{line.slice(magicPS1Placeholder.length)}
											</span>
										) : (
											line
										)}
										{i === output.split('\n').length - 1 ? '' : <br />}
									</div>
								))}
								{loading && (
									<Spinner
										appearance="inverted"
										label="Executing..."
										className={styles.terminalSpinner}
									/>
								)}
								{PS1}
								<input
									id={inputId}
									className={styles.terminalInput}
									placeholder="Type your command here"
									area-label="inline"
									type="text"
									width="100%"
									autoFocus={true}
									disabled={loading}
									onKeyDown={(e) => {
										if (e.key === 'Enter') {
											runCmd(e.currentTarget.value);
											e.currentTarget.value = '';
										}
									}}
								/>
							</TerminalArea>
						)}
					</Card>
				</div>
			</div>
		</>
	);
}
