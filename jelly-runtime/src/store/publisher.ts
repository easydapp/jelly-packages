export type PublisherAnchor = string; // publisher key

export interface Publisher {
    anchor: PublisherAnchor;

    avatar: string;
    name: string;
    bio: string;
    social: string;
}
