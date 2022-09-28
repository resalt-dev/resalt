const appName = 'Resalt'; // THESE ARE NOT CONFIGS - they are constants
const basePath = '/resalt';
const themeColors = [
    'red',
    'volcano',
    'orange',
    'gold',
    'yellow',
    'green',
    'cyan',
    'blue',
    // "geekblue", // Disabled due to too similar to primary color (navy/dark blue)
    'primary',
    'purple',
    'magenta',
];
const githubUrl = 'https://github.com/resalt-dev/resalt';

const apiUrl = `${basePath}/api/1`;

export default {
    apiUrl, appName, basePath, themeColors, githubUrl,
};
