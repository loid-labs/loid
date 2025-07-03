import test from 'ava'

import { EventBuilder, Impact} from '../index.js'

test('build basic event', (t) => {
  const builder = new EventBuilder('test', null);
  const event = builder.withImpact(Impact.Negligible).build()
  t.is(event.impact, Impact.Negligible)
})
