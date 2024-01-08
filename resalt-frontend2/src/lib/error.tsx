import { Toast, ToastBody, ToastTitle, useToastController } from '@fluentui/react-components';

export function showError(title: string, err: Error) {
	console.error(title, err);
	useToastController().dispatchToast(
		<Toast>
			<ToastTitle>{title}</ToastTitle>
			<ToastBody subtitle="Subtitle">err.message</ToastBody>
		</Toast>,
		{ intent: 'error' },
	);
}
