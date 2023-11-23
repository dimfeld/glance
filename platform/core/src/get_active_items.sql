-- Autogenerated by sqlweld

SELECT items.id, items.app_id, items.html, items.charts, items.data,
       items.updated, items.dismissible, items.active,
      jsonb_agg(jsonb_build_object(
          'id', noti.id,
          'html', noti.html,
          'icon', noti.icon,
          'active', noti.active
      )) as notifications
  FROM items
  LEFT JOIN item_notifications noti
      ON items.id = noti.item_id AND items.app_id = noti.app_id AND noti.active
      WHERE items.active = true
  GROUP BY items.id, items.app_id


