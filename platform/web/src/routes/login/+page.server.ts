import { fail, redirect } from '@sveltejs/kit';
import { forwardToApi, type FormResponse } from 'filigree-web';

export const actions = {
  default: async (event) => {
    let response = await forwardToApi('POST', 'auth/login', event);

    if (response.status === 200) {
      redirect(301, '/');
    }

    if (response.status === 401) {
      fail(401, (await response.json()) satisfies FormResponse<{ email: string }>);
    }

    return {
      form: { email: '' },
      error: {},
    } satisfies FormResponse<{ email: string }>;
  },
};
