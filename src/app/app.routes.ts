import { Routes } from '@angular/router';
import { Home } from "./home/home";
import { Touzi } from "./touzi/touzi";

export const routes: Routes = [
	{
		path: "",
		component: Home
	},
	{
		path: "touzi",
		component: Touzi
	}
];
