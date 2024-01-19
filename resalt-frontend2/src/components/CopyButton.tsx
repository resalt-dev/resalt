import { Button, Tooltip, tokens } from '@fluentui/react-components';
import { ClipboardLinkFilled, ClipboardLinkRegular, bundleIcon } from '@fluentui/react-icons';
import { ToastController } from '../lib/toast';

const CopyIcon = bundleIcon(ClipboardLinkFilled, ClipboardLinkRegular);

export function CopyButton(props: {
	name: string;
	value: string;
	toastController: ToastController;
}) {
	const copy = () => {
		if (!props.value) {
			props.toastController.warning(`Nothing to copy`, `No value for "${props.name}'`);
		}
		navigator.clipboard.writeText(props.value + '');
		props.toastController.success(`Copied ${props.name}`);
	};
	return (
		<Tooltip content="Copy" relationship="label">
			<Button
				appearance="subtle"
				size="small"
				icon={<CopyIcon />}
				onClick={copy}
				style={{ marginLeft: tokens.spacingHorizontalXS }}
			/>
		</Tooltip>
	);
}
