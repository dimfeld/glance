import type { HandleClientError } from '@sveltejs/kit';

const errorHandler: HandleClientError = ({ error, event, message, status }) => {
  console.dir(error);
  return {
    status,
    message,
    error,
  };
};

export const handleError = errorHandler;
