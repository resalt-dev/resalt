import React from 'react';
import ReactDOM from 'react-dom/client';
import './lib/fluentui.css';
import { MainRouter } from './routes/Router';

const root = document.getElementById('root');
if (!root) {
	alert('No root element!');
	throw new Error('No root element!');
}

ReactDOM.createRoot(root).render(
	<React.StrictMode>
		<MainRouter />
	</React.StrictMode>,
);
