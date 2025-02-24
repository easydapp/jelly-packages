import { TimestampMills } from '../../../common/time';

export interface AccessDuration {
    start?: TimestampMills; // Start time, including
    end?: TimestampMills; // End time, not included
}

export interface VerifiedAccessDuration {
    start?: TimestampMills; // Start time, including
    end?: TimestampMills; // End time, not included

    // =============== Verification information ===============
}
