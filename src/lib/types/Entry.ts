export interface Entry {
  id: string;
  object_id: string;
  occurred_at: string;
  categoryName: string;
  categoryIcon: string;
  categoryTheme: 'green' | 'blue' | 'pink' | 'orange' | 'purple';
  time: string;
  content: string;
  images: string[];
  tags: string[];
  reminderText: string;
}
