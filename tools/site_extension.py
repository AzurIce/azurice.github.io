"""Build-scoped Notist site extension JSON Lines transport."""
import json
import sys


def serve(handler):
    for line in sys.stdin:
        request = {}
        try:
            request = json.loads(line)
            if not isinstance(request, dict):
                request = {}
                raise ValueError("request must be an object")
            if request.get("version") != 2 or request.get("event") not in {"before_compile", "before_template"}:
                raise ValueError("unsupported Site extension protocol")
            response = handler(request)
        except Exception as error:
            response = {"errors": [str(error)]}
        response.update(version=2, event=request.get("event", "before_compile"))
        print(json.dumps(response, ensure_ascii=False), flush=True)
