/* eslint-disable no-unused-vars */

// eslint-disable-next-line no-shadow
export enum Color {
    red1 = '#fff1f0',
    red2 = '#ffccc7',
    red3 = '#ffa39e',
    red4 = '#ff7875',
    red5 = '#ff4d4f',
    red6 = '#f5222d',
    red7 = '#cf1322',
    red8 = '#a8071a',
    red9 = '#820014',
    red10 = '#5c0011',

    volcano1 = '#fff2e8',
    volcano2 = '#ffd8bf',
    volcano3 = '#ffbb96',
    volcano4 = '#ff9c6e',
    volcano5 = '#ff7a45',
    volcano6 = '#fa541c',
    volcano7 = '#d4380d',
    volcano8 = '#ad2102',
    volcano9 = '#871400',
    volcano10 = '#610b00',

    orange1 = '#fff7e6',
    orange2 = '#ffe7ba',
    orange3 = '#ffd591',
    orange4 = '#ffc069',
    orange5 = '#ffa940',
    orange6 = '#fa8c16',
    orange7 = '#d46b08',
    orange8 = '#ad4e00',
    ornage9 = '#873800',
    orange10 = '#612500',

    gold1 = '#fffbe6',
    gold2 = '#fff1b8',
    gold3 = '#ffe58f',
    gold4 = '#ffd666',
    gold5 = '#ffc53d',
    gold6 = '#faad14',
    gold7 = '#d48806',
    gold8 = '#ad6800',
    gold9 = '#874d00',
    gold10 = '#613400',

    yellow1 = '#feffe6',
    yellow2 = '#ffffb8',
    yellow3 = '#fffb8f',
    yellow4 = '#fff566',
    yellow5 = '#ffec3d',
    yellow6 = '#fadb14',
    yellow7 = '#d4b106',
    yellow8 = '#ad8b00',
    yellow9 = '#876800',
    yellow10 = '#614700',

    lime1 = '#fcffe6',
    lime2 = '#f4ffb8',
    lime3 = '#eaff8f',
    lime4 = '#d3f261',
    lime5 = '#bae637',
    lime6 = '#a0d911',
    lime7 = '#7cb305',
    lime8 = '#5b8c00',
    lime9 = '#3f6600',
    lime10 = '#254000',

    green1 = '#f6ffed',
    green2 = '#d9f7be',
    green3 = '#b7eb8f',
    green4 = '#95de64',
    green5 = '#73d13d',
    green6 = '#52c41a',
    green7 = '#389e0d',
    green8 = '#237804',
    green9 = '#135200',
    green10 = '#092b00',

    cyan1 = '#e6fffb',
    cyan2 = '#b5f5ec',
    cyan3 = '#87e8de',
    cyan4 = '#5cdbd3',
    cyan5 = '#36cfc9',
    cyan6 = '#13c2c2',
    cyan7 = '#08979c',
    cyan8 = '#006d75',
    cyan9 = '#00474f',
    cyan10 = '#002329',

    blue1 = '#e6f7ff',
    blue2 = '#bae7ff',
    blue3 = '#91d5ff',
    blue4 = '#69c0ff',
    blue5 = '#40a9ff',
    blue6 = '#1890ff',
    blue7 = '#096dd9',
    blue8 = '#0050b3',
    blue9 = '#003a8c',
    blue10 = '#002766',

    purple1 = '#f9f0ff',
    purple2 = '#efdbff',
    purple3 = '#d3adf7',
    purple4 = '#b37feb',
    purple5 = '#9254de',
    purple6 = '#722ed1',
    purple7 = '#531dab',
    purple8 = '#391085',
    purple9 = '#22075e',
    purple10 = '#120338',

    magenta1 = '#fff0f6',
    magenta2 = '#ffd6e7',
    magenta3 = '#ffadd2',
    magenta4 = '#ff85c0',
    magenta5 = '#f759ab',
    magenta6 = '#eb2f96',
    magenta7 = '#c41d7f',
    magenta8 = '#9e1068',
    magenta9 = '#780650',
    magenta10 = '#520339',

    gray1 = '#ffffff',
    gray2 = '#fafafa',
    gray3 = '#f5f5f5',
    gray4 = '#f0f0f0',
    gray5 = '#d9d9d9',
    gray6 = '#bfbfbf',
    gray7 = '#8c8c8c',
    gray8 = '#595959',
    gray9 = '#434343',
    gray10 = '#262626',
    gray11 = '#1f1f1f',
    gray12 = '#141414',
    gray13 = '#000000',

    red = red6,
    volcano = volcano6,
    orange = orange6,
    gold = gold6,
    yellow = yellow6,
    lime = lime6,
    green = green6,
    cyan = cyan6,
    blue = blue6,
    purple = purple6,
    magenta = magenta6,

    lightred = red2,
    lightvolcano = volcano2,
    lightorange = orange2,
    lightgold = gold2,
    lightyellow = yellow2,
    lightlime = lime2,
    lightgreen = green2,
    lightcyan = cyan2,
    lightblue = blue2,
    lightpurple = purple2,
    lightmagenta = magenta2,
    lightgray = gray5,

    darkred = red8,
    darkvolcano = volcano8,
    darkorange = orange8,
    darkgold = gold8,
    darkyellow = yellow8,
    darklime = lime8,
    darkgreen = green8,
    darkcyan = cyan8,
    darkblue = blue8,
    darkpurple = purple8,
    darkmagenta = magenta8,
    darkgray = gray11,

    white = gray1,
    muted = gray7,
    gray = gray9,
    black = gray13,

    // Boostrap Colors
    primary = blue8,
    secondary = gray,
    success = green,
    info = cyan,
    warning = yellow,
    danger = red,
    light = lightgray,
    dark = black,

    lightprimary = blue4,
    lightsecondary = lightgray,
    lightsuccess = lightgreen,
    lightinfo = lightcyan,
    lightwarning = lightyellow,
    lightdanger = lightred,

    darkprimary = blue10,
    darksecondary = darkgray,
    darksuccess = darkgreen,
    darkinfo = darkcyan,
    darkwarning = darkyellow,
    darkdanger = darkred,
}

// eslint-disable-next-line no-redeclare
export namespace Color {
    export function getColorValueByName(name: string): string {
        return (Color as any)[name as keyof typeof Color];
    }
}

export default Color;
