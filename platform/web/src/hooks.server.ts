export async function handle({ event, resolve }) {
  return resolve(event);
}

export function handleError({ error, event, message, status }) {
  return {
    status,
    message,
  };
}