import type RunCommand from './RunCommand';

export default class RunResult {
	command: RunCommand;

	num: number;

	data: any;

	constructor(command: RunCommand, num: number, data: any) {
		this.command = command;
		this.num = num;
		this.data = data;
	}
}
