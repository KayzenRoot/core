"""Fail-closed M04 canonical source and old-LegacyProvider authorization regression."""
import json
import subprocess
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
LOCK = '.engineering/context-locks/CORE-WO-M04-001.json'
EVID = '.engineering/evidence/CORE-WO-M04-001.json'
def git_blob(path):
    """Get a tracked source's filter-aware Git blob SHA for exact lock checks."""
    return subprocess.run(['git','hash-object',f'--path={path}',path],cwd=ROOT,check=True,capture_output=True,text=True).stdout.strip()
class StandaloneCanonicalLockTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        """Load the one exact candidate's lock, evidence, and GEF state."""
        cls.lock=json.loads((ROOT/LOCK).read_text(encoding='utf-8'))
        cls.evid=json.loads((ROOT/EVID).read_text(encoding='utf-8'))
        cls.gef=json.loads((ROOT/'.engineering/gef/GEF-CURRENT.json').read_text(encoding='utf-8'))
    def test_exact_nine_sources(self):
        """Require matching live source fingerprints and reject pre-cutover M04 SHA."""
        src=self.lock['canonicalSources']
        self.assertEqual(len(src),9)
        self.assertEqual(src,self.evid['sourceCheck']['candidateCanonicalSourceBlobShas'])
        for p,sha in src.items():
            with self.subTest(path=p): self.assertEqual(sha,git_blob(p))
        self.assertNotEqual(src['docs/modules/M04-RUN-ATTEMPT-STEP-ENGINE.md'],'337db097ec5a7a856d4d7b34f108ebbc149ad04c')
    def test_old_legacy_provider_lock_never_authorizes(self):
        """Keep superseded M04 execution disabled across lock and GEF routing."""
        self.assertEqual(self.lock['status'],'STALE')
        self.assertIs(self.lock['productImplementationAuthorized'],False)
        self.assertEqual(self.lock['executionStatus'],'BLOCKED_RE_ADMISSION')
        self.assertEqual(self.lock['cutoverDecision'],'CORE-D-205')
        self.assertTrue(self.lock['externalV1ConsumersUnknownBlocking'])
        self.assertNotIn('legacy_provider',self.lock['upstream'])
        self.assertEqual(self.lock['historicalAuthorization']['authorizedBase'],'f6b422be5465d5a93d0b8fcf4c9507c205663072')
        self.assertIs(self.gef['productImplementationAuthorized'],False)
        self.assertIsNone(self.gef['activeWorkOrder'])
    def test_evidence_fingerprints(self):
        """Bind blocked M04 evidence to the current Work Order and lock blobs."""
        self.assertEqual(self.evid['status'],'BLOCKED_RE_ADMISSION')
        self.assertEqual(self.evid['contextLock']['status'],'STALE')
        self.assertIs(self.evid['contextLock']['productImplementationAuthorized'],False)
        self.assertEqual(self.evid['sourceCheck']['workOrderBlobSha'],git_blob(self.lock['workOrderSource']['path']))
        self.assertEqual(self.evid['contextLock']['blobSha'],git_blob(LOCK))
        self.assertIs(self.evid['ownerSelfAudit']['independent'],False)
if __name__ == '__main__': unittest.main()
