const PREFIX = 'app';

let counter = 0;

export function newId(name?: string) {
  if (name) {
    return `${PREFIX}-${name}-${++counter}`;
  } else {
    return `${PREFIX}-${++counter}`;
  }
}
