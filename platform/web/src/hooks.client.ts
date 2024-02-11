export function handleError({ error, event, message, status }) {
  return {
    status,
    message,
  };
}
