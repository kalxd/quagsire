import { Component } from '@angular/core';
import { ActionResult, UiContainer, UiTaskDirective } from "drifloon";
import * as R from "rxjs";

const rand = (): number => {
	return Math.floor(Math.random() * 6) + 1;
};

interface TouziResult {
	a: number;
	b: number;
}

@Component({
	selector: 'app-touzi',
	imports: [UiTaskDirective, UiContainer],
	templateUrl: './touzi.html',
	styleUrl: './touzi.css',
})
export class Touzi {
	protected result$: R.Observable<ActionResult<TouziResult>>;

	private start$ = new R.Subject<void>();

	constructor() {
		this.result$ = this.start$
			.pipe(
				ActionResult.exhaustMap(_ => {
					const a = rand();
					const b = rand();
					return R.timer(200).pipe(R.map(_ => ({ a, b })));
				}),
				R.startWith(ActionResult.Ok({
					a: 0,
					b: 0
				}))
			);
	}

	startTouzi(): void {
		this.start$.next();
	}
}
