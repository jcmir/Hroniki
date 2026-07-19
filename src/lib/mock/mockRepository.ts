export const mockCategories = [
  { id: 'cat-1', name: 'Сад' },
  { id: 'cat-2', name: 'Здоровье' },
  { id: 'cat-3', name: 'Авто' }
];

export let mockObjects = [
  { id: 'obj-1', category_id: 'cat-1', name: 'Яблоня (Белый налив)', description: 'Посажена в мае 2024 года.' }
];

export let mockEntries = [
  { id: 'ent-1', object_id: 'obj-1', occurred_at: new Date().toISOString(), title: 'Полив яблони', description: 'Полила 2 ведрами теплой воды' }
];

export const mockPhotosMap = new Map<string, any[]>();

export function mockInvoke(cmd: string, args: any): any {
  console.warn(`[Tauri Mock] Invoke '${cmd}'`, args);
  if (cmd === 'get_categories') {
    return mockCategories;
  }
  if (cmd === 'get_objects') {
    return mockObjects;
  }
  if (cmd === 'get_entries') {
    return mockEntries;
  }
  if (cmd === 'get_entry_photos') {
    return mockPhotosMap.get(args.entryId) || [];
  }
  if (cmd === 'search_entries') {
    let results = [...mockEntries];
    if (args.queryText) {
      const q = args.queryText.toLowerCase();
      results = results.filter(e => e.title.toLowerCase().includes(q) || (e.description && e.description.toLowerCase().includes(q)));
    }
    if (args.objectId) {
      results = results.filter(e => e.object_id === args.objectId);
    }
    return results;
  }
  if (cmd === 'get_object_stats') {
    const objEntries = mockEntries.filter(e => e.object_id === args.objectId);
    return {
      age_days: 12,
      total_entries: objEntries.length,
      total_photos: objEntries.length,
      last_event_title: objEntries[0] ? objEntries[0].title : null,
      last_event_date: objEntries[0] ? objEntries[0].occurred_at : null,
      next_reminder_date: new Date(Date.now() + 86400000 * 14).toISOString()
    };
  }
  if (cmd === 'create_object') {
    const newObj = {
      id: `obj-${Math.random()}`,
      category_id: args.categoryId,
      name: args.name,
      description: args.description || null
    };
    mockObjects.push(newObj);
    return newObj.id;
  }
  if (cmd === 'create_entry') {
    const newEnt = {
      id: `ent-${Math.random()}`,
      object_id: args.objectId,
      occurred_at: new Date().toISOString(),
      title: args.title,
      description: args.description || null
    };
    mockEntries.unshift(newEnt);
    if (args.imageFilenames && args.imageFilenames.length > 0) {
      const mockPhotosList = args.imageFilenames.map((f: string) => ({
        id: `p-${Math.random()}`,
        entry_id: newEnt.id,
        path: f,
        thumbnail: f
      }));
      mockPhotosMap.set(newEnt.id, mockPhotosList);
    }
    return newEnt.id;
  }
  if (cmd === 'select_images') {
    return ['/garden_tomatoes.png'];
  }
  if (cmd === 'save_media') {
    return args.sourcePath;
  }
  if (cmd === 'delete_entry') {
    mockEntries = mockEntries.filter(e => e.id !== args.entryId);
    mockPhotosMap.delete(args.entryId);
    return null;
  }
  if (cmd === 'update_entry') {
    const ent = mockEntries.find(e => e.id === args.entryId);
    if (ent) {
      ent.title = args.title;
      ent.description = args.description;
    }
    return null;
  }
  if (cmd === 'create_reminder') {
    return `mock-reminder-${Math.random()}`;
  }
  if (cmd === 'get_reminders') {
    return [];
  }
  if (cmd === 'complete_reminder') {
    return null;
  }
  if (cmd === 'snooze_reminder') {
    return null;
  }
  if (cmd === 'is_pin_configured') {
    if (typeof window !== 'undefined') {
      return !!localStorage.getItem('hroniki_mock_pin');
    }
    return false;
  }
  if (cmd === 'set_pin') {
    if (typeof window !== 'undefined') {
      localStorage.setItem('hroniki_mock_pin', args.pin);
    }
    return null;
  }
  if (cmd === 'verify_pin') {
    if (typeof window !== 'undefined') {
      return localStorage.getItem('hroniki_mock_pin') === args.pin;
    }
    return false;
  }
  if (cmd === 'disable_pin') {
    if (typeof window !== 'undefined') {
      localStorage.removeItem('hroniki_mock_pin');
    }
    return null;
  }
  if (cmd === 'export_archive') {
    return "Mock export completed";
  }
  if (cmd === 'import_archive') {
    return null;
  }
  if (cmd === 'is_onboarding_completed') {
    if (typeof window !== 'undefined') {
      return localStorage.getItem('hroniki_mock_onboarding_completed') === 'true';
    }
    return false;
  }
  if (cmd === 'complete_onboarding') {
    if (typeof window !== 'undefined') {
      localStorage.setItem('hroniki_mock_onboarding_completed', 'true');
      localStorage.setItem('hroniki_mock_username', args.username);
    }
    return null;
  }
  if (cmd === 'seed_demo_data') {
    return null;
  }
  if (cmd === 'get_username') {
    if (typeof window !== 'undefined') {
      return localStorage.getItem('hroniki_mock_username') || 'Александр';
    }
    return 'Александр';
  }
  return null;
}
