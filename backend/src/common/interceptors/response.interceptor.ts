import {
  CallHandler,
  ExecutionContext,
  Injectable,
  NestInterceptor,
  StreamableFile,
} from '@nestjs/common';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';
import type { Response } from 'express';

export interface ApiSuccessEnvelope<T = unknown> {
  success: true;
  data: T;
  timestamp: string;
}

const NO_BODY_STATUS_CODES = new Set([204, 304]);

@Injectable()
export class ResponseInterceptor implements NestInterceptor {
  intercept(context: ExecutionContext, next: CallHandler): Observable<unknown> {
    const http = context.switchToHttp();
    const response = http.getResponse<Response>();

    return next.handle().pipe(
      map((data: unknown) => {
        if (data instanceof StreamableFile) {
          return data;
        }
        if (NO_BODY_STATUS_CODES.has(response.statusCode)) {
          return data;
        }
        const envelope: ApiSuccessEnvelope = {
          success: true,
          data,
          timestamp: new Date().toISOString(),
        };
        return envelope;
      }),
    );
  }
}
