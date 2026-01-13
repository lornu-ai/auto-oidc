
import React from 'react';

export const COMPONENTS = {
  USER: {
    id: 'user',
    name: 'End User / Bot',
    description: 'The initiator of the OIDC request or automated task.',
    category: 'User'
  },
  ZERO_TRUST: {
    id: 'zt',
    name: 'Cloudflare Zero Trust',
    description: 'Enforces identity-based access policies and TLS inspection.',
    category: 'Edge'
  },
  WORKER_RELAY: {
    id: 'worker',
    name: 'Cloudflare Worker (The Relay)',
    description: 'Distributed OIDC proxy that mutates headers and verifies JWTs.',
    category: 'Edge'
  },
  IAM: {
    id: 'iam',
    name: 'Central IAM (Auth0/Okta)',
    description: 'The Source of Truth for identity and distributed claims.',
    category: 'Identity'
  },
  HUB_AUTOMATION: {
    id: 'automation_hub',
    name: 'automation-hub.dockworker.ai',
    description: 'Service-to-service automation and orchestration endpoint.',
    category: 'Resource'
  },
  HUB_OIDC: {
    id: 'oidc_hub',
    name: 'oidc-hub.dockworker.ai',
    description: 'User-facing identity federation and SSO landing.',
    category: 'Resource'
  }
};

export const FLOWS = [
  { from: 'user', to: 'zt', label: '1. Request', description: 'User hits the subdomain via Cloudflare edge.' },
  { from: 'zt', to: 'iam', label: '2. Challenge', description: 'Zero Trust redirects to IAM for primary OIDC login.' },
  { from: 'iam', to: 'zt', label: '3. Token', description: 'Identity provider returns signed JWT to Access.' },
  { from: 'zt', to: 'worker', label: '4. Forward', description: 'Cloudflare Access forwards the authenticated request to the Worker.' },
  { from: 'worker', to: 'automation_hub', label: '5. Relay', description: 'Worker injects custom claims and routes to specific backend.' }
];
