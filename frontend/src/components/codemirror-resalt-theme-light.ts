import { EditorView } from '@codemirror/view';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { tags as t } from '@lezer/highlight';

// Same between Light and Dark themes
const regex = 'var(--cyan)';
const invalid = 'var(--white)';
const bracket = 'var(--gray-7)';
const label = 'var(--blue-4)';
const darkBackground = 'var(--gray-12)';
const tooltipBackground = 'var(--gray-10)';
const cursor = 'var(--primary)';
const field = 'var(--red)';
const bool = 'var(--orange)';
const keyword = 'var(--magenta)';

// Unique to Light theme
const str = 'var(--primary)';
const background = 'var(--white)';
const highlightBackground = 'var(--gray-4)';
const selection = 'var(--blue-2)';
const rownum = 'var(--blue)';
const num = 'var(--green-7)';

export const resaltLightTheme = EditorView.theme(
    {
        '&': {
            color: bracket,
            backgroundColor: background,
        },

        '.cm-content': {
            caretColor: cursor,
        },

        '.cm-cursor, .cm-dropCursor': { borderLeftColor: cursor },
        '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection':
            { backgroundColor: selection },

        '.cm-panels': { backgroundColor: darkBackground, color: bracket },
        '.cm-panels.cm-panels-top': { borderBottom: '2px solid black' },
        '.cm-panels.cm-panels-bottom': { borderTop: '2px solid black' },

        '.cm-searchMatch': {
            backgroundColor: '#72a1ff59',
            outline: '1px solid #457dff',
        },
        '.cm-searchMatch.cm-searchMatch-selected': {
            backgroundColor: '#6199ff2f',
        },

        '.cm-activeLine': { backgroundColor: highlightBackground },
        '.cm-selectionMatch': { backgroundColor: '#aafe661a' },

        '&.cm-focused .cm-matchingBracket, &.cm-focused .cm-nonmatchingBracket':
            {
                backgroundColor: '#bad0f847',
                outline: '1px solid #515a6b',
            },

        '.cm-gutters': {
            backgroundColor: background,
            color: rownum,
            border: 'none',
        },

        '.cm-activeLineGutter': {
            backgroundColor: highlightBackground,
        },

        '.cm-foldPlaceholder': {
            backgroundColor: 'transparent',
            border: 'none',
            color: '#ddd',
        },

        '.cm-tooltip': {
            border: 'none',
            backgroundColor: tooltipBackground,
        },
        '.cm-tooltip .cm-tooltip-arrow:before': {
            borderTopColor: 'transparent',
            borderBottomColor: 'transparent',
        },
        '.cm-tooltip .cm-tooltip-arrow:after': {
            borderTopColor: tooltipBackground,
            borderBottomColor: tooltipBackground,
        },
        '.cm-tooltip-autocomplete': {
            '& > ul > li[aria-selected]': {
                backgroundColor: highlightBackground,
                color: bracket,
            },
        },
    },
    { dark: true },
);

export const resaltLightHighlightStyle = HighlightStyle.define([
    { tag: t.keyword, color: keyword },
    {
        tag: [t.name, t.deleted, t.character, t.propertyName, t.macroName],
        color: field,
    },
    { tag: [t.function(t.variableName), t.labelName], color: label },
    { tag: [t.color, t.constant(t.name), t.standard(t.name)], color: bool },
    { tag: [t.definition(t.name), t.separator], color: bracket },
    {
        tag: [
            t.typeName,
            t.className,
            t.number,
            t.changed,
            t.annotation,
            t.modifier,
            t.self,
            t.namespace,
        ],
        color: num,
    },
    {
        tag: [
            t.operator,
            t.operatorKeyword,
            t.url,
            t.escape,
            t.regexp,
            t.link,
            t.special(t.string),
        ],
        color: regex,
    },
    { tag: [t.meta, t.comment], color: rownum },
    { tag: t.strong, fontWeight: 'bold' },
    { tag: t.emphasis, fontStyle: 'italic' },
    { tag: t.strikethrough, textDecoration: 'line-through' },
    { tag: t.link, color: rownum, textDecoration: 'underline' },
    { tag: t.heading, fontWeight: 'bold', color: field },
    { tag: [t.atom, t.bool, t.special(t.variableName)], color: bool },
    { tag: [t.processingInstruction, t.string, t.inserted], color: str },
    { tag: t.invalid, color: invalid },
]);

export const resaltLight = [
    resaltLightTheme,
    syntaxHighlighting(resaltLightHighlightStyle),
];
