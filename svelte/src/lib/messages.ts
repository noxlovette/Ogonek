/**
 * Centralized notification messages for the entire application
 * Accessible via message.mnotificationText() pattern for consistency
 */

import { m } from "./paraglide/messages";

export const notificationMessages = {
  // Auth messages
  auth: {
    loginSuccess: m.upper_formal_mammoth_gaze(),
    logoutSuccess: m.shy_small_reindeer_nourish(),
    signupSuccess: m.noble_best_insect_delight(),
    passwordChanged: "Password updated",
    invalidCredentials: "Invalid username or password",
    signupFailure: "Signup failed",
    sessionExpired: "Session expired, please log in again",
    accountLocked: "Account temporarily locked",
  },

  // CRUD operations
  crud: {
    created: m.created(),
    updated: m.changesSaved(),
    deleted: m.tiny_happy_rat_bless(),
    restored: "Restored successfully",
    duplicated: "Duplicated successfully",
    archived: "Archived successfully",
    published: "Published successfully",
    unpublished: "Unpublished successfully",
  },

  // Task-specific messages
  tasks: {
    completed: m.markedAsCompleted(),
    notCompleted: m.notCompleted(),
    assigned: "Task assigned",
    unassigned: "Task unassigned",
    overdue: "Task is overdue",
    dueSoon: "Task due soon",
    priorityChanged: "Task priority updated",
    teacherNotified: m.minor_mad_hare_buzz(),
  },

  // File operations
  files: {
    uploaded: "File uploaded successfully",
    deleted: "File deleted",
    downloadStarted: "Download started",
    uploadFailed: "Upload failed",
    fileTooLarge: "File size exceeds limit",
    invalidFormat: "Invalid file format",
    processingFile: "Processing file...",
  },

  // Flashcard/Deck operations
  flashcards: {
    subscribed: m.stout_royal_macaw_fear(),
    unsubscribed: m.elegant_small_gadfly_quell(),
    cardAdded: "Card added",
    cardRemoved: "Card removed",
    sessionComplete: "Study session complete",
    progressSaved: "Progress saved",
  },

  // Student/Teacher operations
  students: {
    added: "Student added",
    removed: "Student removed",
    invited: "Student invited",
    inviteResent: "Invitation resent",
    profileUpdated: "Student profile updated",
    assignmentCreated: "Assignment created",
  },

  lessons: {
    scheduled: "Lesson scheduled",
    rescheduled: "Lesson rescheduled",
    cancelled: "Lesson cancelled",
    started: "Lesson started",
    ended: "Lesson ended",
  },

  // Settings & Preferences
  settings: {
    saved: "Settings saved",
    restored: "Settings restored to default",
    preferencesUpdated: "Preferences updated",
    themeChanged: "Theme changed",
    languageChanged: "Language changed",
    notificationsEnabled: "Notifications enabled",
    notificationsDisabled: "Notifications disabled",
  },

  // Notifications
  notifications: {
    deviceRegistered: "Device registered for notifications",
    deviceRemoved: "Device removed from notifications",
    notificationSent: "Notification sent",
    homeworkRequested: "Homework request sent",
    reminderSet: "Reminder set",
  },

  // Error messages
  errors: {
    generic: "Something went wrong",
    networkError: "Network connection error",
    serverError: "Server error occurred",
    validationFailed: "Please check your input",
    permissionDenied: "Permission denied",
    resourceNotFound: "Resource not found",
    operationFailed: "Operation failed",
    saveError: "Failed to save changes",
    loadError: "Failed to load data",
    connectionLost: "Connection lost",
    timeoutError: "Request timed out",
  },

  // Success confirmations
  confirmations: {
    dataSaved: "Data saved successfully",
    emailSent: "Email sent",
    inviteSent: "Invitation sent",
    backupCreated: "Backup created",
    importComplete: "Import completed",
    exportComplete: "Export completed",
    syncComplete: "Sync completed",
  },

  // Warning messages
  warnings: {
    unsavedChanges: "You have unsaved changes",
    actionCannotBeUndone: "This action cannot be undone",
    dataWillBeLost: "Data will be lost",
    largeFileSelected: "Large file selected, upload may take time",
    lowStorage: "Storage space running low",
    maintenanceMode: "System maintenance in progress",
  },

  // Info messages
  info: {
    loading: "Loading...",
    processing: "Processing...",
    saving: "Saving...",
    uploading: "Uploading...",
    downloading: "Downloading...",
    syncing: "Syncing...",
    searching: "Searching...",
    connecting: "Connecting...",
    preparingData: "Preparing data...",
    almostDone: "Almost done...",
  },
} as const;

// Type-safe message accessor
type NotificationPath =
  | `auth.${keyof typeof notificationMessages.auth}`
  | `crud.${keyof typeof notificationMessages.crud}`
  | `tasks.${keyof typeof notificationMessages.tasks}`
  | `files.${keyof typeof notificationMessages.files}`
  | `flashcards.${keyof typeof notificationMessages.flashcards}`
  | `students.${keyof typeof notificationMessages.students}`
  | `lessons.${keyof typeof notificationMessages.lessons}`
  | `settings.${keyof typeof notificationMessages.settings}`
  | `notifications.${keyof typeof notificationMessages.notifications}`
  | `errors.${keyof typeof notificationMessages.errors}`
  | `confirmations.${keyof typeof notificationMessages.confirmations}`
  | `warnings.${keyof typeof notificationMessages.warnings}`
  | `info.${keyof typeof notificationMessages.info}`;

/**
 * Type-safe notification message getter
 * Usage: message.mnotificationText('auth.welcomeHome')
 */
export function getNotificationText(path: NotificationPath): string {
  const keys = path.split(".") as [keyof typeof notificationMessages, string];
  const category = notificationMessages[keys[0]] as Record<string, string>;
  return category[keys[1]] || "Message not found";
}

export const message = {
  notificationText: getNotificationText,

  auth: notificationMessages.auth,
  crud: notificationMessages.crud,
  tasks: notificationMessages.tasks,
  files: notificationMessages.files,
  flashcards: notificationMessages.flashcards,
  students: notificationMessages.students,
  lessons: notificationMessages.lessons,
  settings: notificationMessages.settings,
  notifications: notificationMessages.notifications,
  errors: notificationMessages.errors,
  confirmations: notificationMessages.confirmations,
  warnings: notificationMessages.warnings,
  info: notificationMessages.info,
};

export default message;
