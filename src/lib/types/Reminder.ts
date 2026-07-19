export interface Reminder {
  id: string;
  entry_id: string;
  trigger_at: string;
  status: 'Scheduled' | 'Snoozed' | 'Completed' | string;
  repeat_days: number | null;
  completed_at: string | null;
}
