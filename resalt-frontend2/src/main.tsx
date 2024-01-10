import React from 'react';
import ReactDOM from 'react-dom/client';
import './lib/fluentui.css';
import { MainRouter } from './routes/Router';

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<MainRouter />
	</React.StrictMode>,
);
