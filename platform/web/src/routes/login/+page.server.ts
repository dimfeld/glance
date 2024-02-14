import { env } from '$env/dynamic/private';
import { error, fail, redirect } from '@sveltejs/kit';
import {
  applyResponseCookies,
  client,
  forwardToApi,
  isExtractedResponse,
  handleFormResponse,
  type FormResponse,
} from 'filigree-web';

interface ActionFormResponseData {
  email: string;
  password?: string;
}
export const actions = {
  login: async (event) => {
    let response = await forwardToApi('POST', 'auth/login', event, { tolerateFailure: true });

    applyResponseCookies(response, event.cookies);

    if (response.ok) {
      redirect(301, '/');
    }

    const result = await handleFormResponse<ActionFormResponseData>(response, [400, 401]);
    if (isExtractedResponse(result)) {
      // We probably should never hit this since we already checked `response.ok` above.
      return result.body;
    }

    if (result.data.form) {
      delete result.data.form.password;
    }
    return result;
  },
  passwordless: async (event) => {
    const res = await forwardToApi('POST', 'auth/email_login', event);
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

interface PasswordlessLoginResult {
  message: string;
  redirect_to?: string;
}

export async function load({ fetch, url, cookies }) {
  let token = url.searchParams.get('token');

  let message: string | undefined;
  if (token) {
    // User is trying to do a passwordless login.

    let res = await client({
      url: '/api/auth/email_login',
      method: 'GET',
      query: url.searchParams,
      fetch,
      tolerateFailure: true,
    });

    applyResponseCookies(res, cookies);
    if (res.ok) {
      let successBody = (await res.json()) as PasswordlessLoginResult;
      return {
        oauthEnabled,
        logInSuccess: true,
        ...successBody,
      };
    } else {
      let response = await res.json();
      message = response.error?.message ?? 'An error occurred';
    }
  }

  return {
    oauthEnabled,
    message,
  };
}
