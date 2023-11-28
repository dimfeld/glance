/* eslint-disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

/**
 * The top-level data for the app
 */
export interface AppData {
  /**
   * An array of data items that the app is publishing
   */
  items?: AppItem[];
  /**
   * The name of the app
   */
  name: string;
  /**
   * The path at which this app is installed
   */
  path: string;
  /**
   * Request that the platform run the app at the specified schedule, if it does not have its own methods of scheduling updates
   */
  schedule?: AppSchedule[];
  /**
   * If false, the app does not keep its own state, so the platform should do a closer diff to see if an item has changed since the last write If true, the app can just check the updated timestamp to see if an item has changed
   */
  stateful?: boolean;
  /**
   * Information only used to render the UI of the app
   */
  ui?: AppUiInfo | null;
}
/**
 * An item published by the app
 */
export interface AppItem {
  /**
   * Display information for the item
   */
  data: AppItemData;
  /**
   * An ID that uniquely identifies this item among others published by the app
   */
  id: string;
  /**
   * Notifications for this item
   */
  notify?: Notification[];
  /**
   * Whether the item can be dismissed by the viewer
   */
  persistent?: boolean;
  /**
   * When the item was last updated
   */
  updated: string;
}
/**
 * Information for an app item
 */
export interface AppItemData {
  /**
   * Extra structured data for use by chart or other formatters
   */
  data?: {
    [k: string]: unknown;
  };
  /**
   * Extra information which can be shown
   */
  detail?: string | null;
  /**
   * An icon to show with this item
   */
  icon?: string | null;
  /**
   * A subtitle to display below the title
   */
  subtitle?: string | null;
  /**
   * The title at the top of the card
   */
  title: string;
}
/**
 * A notification from the app
 */
export interface Notification {
  /**
   * Data for the notification
   */
  data: NotificationData;
  /**
   * A unique ID among other notifications for this app
   */
  id: string;
}
/**
 * Data for a notification
 */
export interface NotificationData {
  /**
   * An icon to show with the notification
   */
  icon?: string | null;
  /**
   * A subtitle to display below the title
   */
  subtitle?: string | null;
  /**
   * The title at the top of the card
   */
  title: string;
}
/**
 * A schedule on which to run this app. This is not implemented yet.
 */
export interface AppSchedule {
  /**
   * Arguments to pass to the app
   */
  arguments?: string[];
  /**
   * The cron schedule for the app
   */
  cron: string;
}
/**
 * Information only used to render the UI of the app
 */
export interface AppUiInfo {
  /**
   * The icon that the app should show (exact format TBD)
   */
  icon?: string | null;
}
