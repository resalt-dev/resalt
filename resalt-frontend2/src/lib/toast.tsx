import { ToastIntent } from '@fluentui/react-components';

export class ToastMessage {
	public readonly title: string;
	public readonly body: string;
	public readonly intent: ToastIntent;

	constructor(title: string, body: string, intent: ToastIntent) {
		this.title = title;
		this.body = body;
		this.intent = intent;
	}
}

export class ToastController {
	private readonly _setToasts: React.Dispatch<React.SetStateAction<ToastMessage[]>>;
	constructor(setToasts: React.Dispatch<React.SetStateAction<ToastMessage[]>>) {
		this._setToasts = setToasts;
	}

	public info(title: string, body?: string) {
		console.log('Toast:INFO', title, body);
		this.dispatchToast(new ToastMessage(title, body || '', 'info'));
	}

	public success(title: string, body?: string) {
		console.log('Toast:SUCCESS', title, body);
		this.dispatchToast(new ToastMessage(title, body || '', 'success'));
	}

	public error(title: string, err?: Error) {
		console.error('Toast:ERROR', title, err);
		this.dispatchToast(new ToastMessage(title, err?.message || '', 'error'));
	}

	public warning(title: string, body?: string) {
		console.warn('Toast:WARNING', title, body);
		this.dispatchToast(new ToastMessage(title, body || '', 'warning'));
	}

	dispatchToast(toast: ToastMessage) {
		this._setToasts((toasts) => [...toasts, toast]);
	}
}
