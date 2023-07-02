import type RunCommand from './RunCommand';

export default class RunResult {
	command: RunCommand;

	num: number;

	data: unknown;

	constructor(command: RunCommand, num: number, data: unknown) {
		this.command = command;
		this.num = num;
		this.data = data;
	}
}
