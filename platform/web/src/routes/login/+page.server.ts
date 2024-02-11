import { error, fail, redirect } from '@sveltejs/kit';
import { type FormResponse, forwardToApi } from 'filigree-web';
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
      fail(response.status, (await response.json()) satisfies ActionResponse);
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
  github: getOauthEnabledFlag('OAUTH_GITHUB_CLIENT_ID'),
  twitter: getOauthEnabledFlag('OAUTH_TWITTER_CLIENT_ID'),
  google: getOauthEnabledFlag('OAUTH_GOOGLE_CLIENT_ID'),
};

export const load = () => {
  return {
    oauthEnabled,
  };
};
