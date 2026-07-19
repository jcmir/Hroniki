export interface Entry {
  id: string;
  object_id: string;
  occurred_at: string;
  title: string;
  description: string | null;
  created_at: string;
  updated_at: string;
  images?: string[];
  tags?: string[];
  reminderText?: string;
}
