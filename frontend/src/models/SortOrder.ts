/* eslint-disable no-unused-vars */

// eslint-disable-next-line no-shadow
export enum SortOrder {
    Up = 'up',
    Down = 'down',
    None = 'none',
}

// eslint-disable-next-line no-redeclare
export namespace SortOrder {
    export function next(order: SortOrder): SortOrder {
        switch (order) {
        case SortOrder.Up:
            return SortOrder.Down;
        case SortOrder.Down:
            return SortOrder.None;
        default:
            return SortOrder.Up;
        }
    }
}
