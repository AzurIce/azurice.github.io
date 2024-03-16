/* @refresh reload */
import { render } from 'solid-js/web'

import './index.css'
import App from './App'

const root = document.getElementById('root')
// import { Router } from "@solidjs/router";

// import Router from "AoikeRouter.tsx";

// render(() => <Router></Router>, root!)
render(() => <App />, root!)
