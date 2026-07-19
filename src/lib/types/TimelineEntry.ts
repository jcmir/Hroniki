import type { Entry } from './Entry';

export interface TimelineEntry extends Omit<Entry, 'title' | 'description' | 'created_at' | 'updated_at'> {
  categoryName: string;
  categoryIcon: string;
  categoryTheme: 'green' | 'blue' | 'pink' | 'orange' | 'purple';
  time: string;
  content: string;
  images: string[];
  tags: string[];
  reminderText: string;
}
