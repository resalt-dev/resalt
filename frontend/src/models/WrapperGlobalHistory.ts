import type { NavigateFn, NavigationAction, NavigatorHistory } from 'svelte-navigator';
import type { Unlisten } from 'svelte-navigator/types/NavigatorHistory';
import type RawLocation from 'svelte-navigator/types/RawLocation';

export default class WrapperGlobalHistory implements NavigatorHistory {
	constructor(
		location: RawLocation,
		listen: (
			listener: ({
				location,
				action,
			}: {
				location: RawLocation;
				action: NavigationAction;
			}) => void,
		) => Unlisten,
		navigate: NavigateFn,
	) {
		this.location = location;
		this.listen = listen;
		this.navigate = navigate;
	}

	/**
	 * The current location
	 */
	readonly location: RawLocation;
	/**
	 * Listen to changes in location.
	 *
	 * @param listener The listener function will be called when the
	 * location changes.
	 * @returns The unlisten function, which can be used to unsubscribe
	 * the listener
	 */
	readonly listen: (
		listener: ({
			location,
			action,
		}: {
			location: RawLocation;
			action: NavigationAction;
		}) => void,
	) => Unlisten;
	/**
	 * Navigate to a new route.
	 * @param to The path to navigate to.
	 *
	 * If `to` is a number we will navigate to the stack entry index + `to`
	 * (-> `navigate(-1)`, is equivalent to hitting the back button of the browser)
	 * @param options Navigation options
	 */
	navigate: NavigateFn;
}
