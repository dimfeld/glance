import { error, fail, redirect } from '@sveltejs/kit';
import { type FormResponse, forwardToApi, client } from 'filigree-web';
import { env } from '$env/dynamic/private';

interface ActionFormResponseData {
  email: string;
  password?: string;
}
type ActionResponse = FormResponse<ActionFormResponseData>;

export const actions = {
  login: async (event) => {
    let response = await forwardToApi('POST', 'auth/login', event, { tolerateFailure: true });

    if (response.ok) {
      redirect(301, '/');
    }

    if (response.status === 400 || response.status === 401) {
      const body = await response.json();
      if (body.form) {
        delete body.form.password;
      }
      return fail(response.status, body satisfies ActionResponse);
    } else {
      error(response.status, {});
    }

    return {
      form: { email: '' } as ActionFormResponseData,
    } satisfies ActionResponse;
  },
  passwordless: async (event) => {
    const res = await forwardToApi('POST', 'auth/email_login', event, { tolerateFailure: true });
    if (!res.ok) {
      const data = await res.json();
      error(500, data);
    }
    return {
      form: { email: '' } as ActionFormResponseData,
      message: 'You should receive an email soon.',
    } satisfies FormResponse<{ email: string; password?: string }>;
  },
};

function getOauthEnabledFlag(varName: string) {
  return env[varName] ? true : undefined;
}

const oauthEnabled = {
  github: getOauthEnabledFlag('GLANCE_OAUTH_GITHUB_CLIENT_ID'),
  twitter: getOauthEnabledFlag('GLANCE_OAUTH_TWITTER_CLIENT_ID'),
  google: getOauthEnabledFlag('GLANCE_OAUTH_GOOGLE_CLIENT_ID'),
};

export const load = async ({ fetch, url }) => {
  let token = url.searchParams.get('token');

  let message: string | undefined;
  if (token) {
    // User is trying to do a passwordless login.
    interface PasswordlessLoginResult {
      message: string;
      redirect_to?: string;
    }

    // TODO improve ergonomics of making a call that might return an error. This probably involves some helper functions
    // that automate some type inference
    let res = await client({
      url: '/api/auth/email_login',
      method: 'GET',
      query: url.searchParams,
      fetch,
      tolerateFailure: true,
    });

    if (res.ok) {
      let body = (await res.json()) as PasswordlessLoginResult;
      redirect(301, body.redirect_to ?? '/');
    } else {
      let response = await res.json();
      message = response.error?.message ?? 'An error occurred';
    }
  }

  return {
    oauthEnabled,
    message,
  };
};
