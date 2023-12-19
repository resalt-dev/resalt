import React from 'react';
import ReactDOM from 'react-dom/client';
import './lib/fluentui.css';
import { FluentProvider, webLightTheme } from '@fluentui/react-components';
import { tokens } from '@fluentui/tokens';
import ResaltHeader from './layout/ResaltHeader';

const styles = {
	fluentProvider: {
		backgroundColor: tokens.colorNeutralBackground4,
	},

	//
	// Body
	//
	bodyGrid: {
		display: 'grid',
		gridTemplateColumns: '[sidebar] 280px [main-area] auto',
		gridTemplateRows: 'auto',
	},
	bodySidebar: {
		gridColumn: 'sidebar',
		height: 'calc(100vh - 48px)',
	},
	mainArea: {
		gridColumn: 'main-area',
		// backgroundColor: tokens.colorNeutralBackground2,
	},
};

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<FluentProvider theme={webLightTheme} style={styles.fluentProvider}>
			<ResaltHeader />
			<div className="m-0" style={styles.bodyGrid}>
				<div style={styles.bodySidebar}>Sidebar</div>
				<div style={styles.mainArea}>App</div>
			</div>
		</FluentProvider>
	</React.StrictMode>,
);
