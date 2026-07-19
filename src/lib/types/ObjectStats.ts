export interface ObjectStats {
  age_days: number;
  total_entries: number;
  total_photos: number;
  last_event_title: string | null;
  last_event_date: string | null;
  next_reminder_date: string | null;
}
