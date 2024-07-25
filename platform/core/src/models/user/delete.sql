DELETE FROM public.users
WHERE id = $1
  AND organization_id = $2
