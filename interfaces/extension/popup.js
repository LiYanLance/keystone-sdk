import init, { parse_sol_signature } from './ur-registry-ffi/ur_registry_ffi.js'

init().then(() => {
  const signatureCBOR = 'a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7'
  console.log('parse_sol_signature in frontend:')
  console.log('signatureCBOR:', signatureCBOR)
  console.log('signature:', parse_sol_signature(signatureCBOR))
})
