
export enum HubType {
  AUTOMATION = 'automation-hub.dockworker.ai',
  OIDC = 'oidc-hub.dockworker.ai'
}

export interface NodePosition {
  x: number;
  y: number;
}

export interface ArchitectureComponent {
  id: string;
  name: string;
  description: string;
  icon: string;
  category: 'User' | 'Edge' | 'Identity' | 'Resource';
}

export interface FlowStep {
  from: string;
  to: string;
  label: string;
  description: string;
}
