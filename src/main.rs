use std::env;
use std::fs;
use std::path::Path;
use std::process;

struct PatchDefinition {
    name: &'static str,
    search: &'static str,
    replace: &'static str,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing file argument.");
        eprintln!("Usage: xolite-patcher <path-to-xoa-deploy.vue>");
        process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    if !file_path.exists() {
        eprintln!("Error: Target file does not exist at {:?}", file_path);
        process::exit(1);
    }

    let mut content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read file: {}", e);
            process::exit(1);
        }
    };

    // Define structural landmarks tailored exactly to the provided xoa-deploy.vue
    let patches = vec![
        PatchDefinition {
            name: "Vue Imports Validation Landmark",
            search: "import { computed, ref } from 'vue'",
            replace: "import { computed, ref, watch } from 'vue'",
        },
        PatchDefinition {
            name: "Configuration Form Elements Injection",
            search: "        <FormSection :label=\"t('configuration')\">",
            replace: r#"        <FormSection :label="t('configuration')">
          <div class="row">
            <VtsInputWrapper :label="t('xoa-image-url')">
              <div class="image-source-row">
                <select v-model="imageSource" class="image-source-select">
                  <option value="xoa-hl">{{ t('xoa-hl-image') }}</option>
                  <option value="vates">{{ t('vates-image') }}</option>
                  <option value="ronivay">{{ t('ronivay-image') }}</option>
                  <option value="custom">{{ t('custom-url') }}</option>
                </select>
                <FormInput
                  v-if="imageSource === 'custom'"
                  v-model="xoaImageUrl"
                  placeholder="https://example.com/image.xva.gz"
                  class="custom-url-input"
                />
              </div>
            </VtsInputWrapper>
          </div>
          <div v-if="imageSource !== 'vates'" class="row">
            <VtsInputWrapper>
              <UiToggle v-model="verifySsl">{{ t('verify-ssl-certificate') }}</UiToggle>
            </VtsInputWrapper>
          </div>"#,
        },
        PatchDefinition {
            name: "Admin User Editable Attributes",
            search: r#"            <VtsInputWrapper
              :label="t('admin-login')"
              learn-more-url="https://docs.xen-orchestra.com/xoa#default-xo-account"
            >
              <FormInput v-model="xoaUser" required placeholder="email@example.com" />
            </VtsInputWrapper>"#,
            replace: r#"            <VtsInputWrapper
              :label="t('admin-login')"
              learn-more-url="https://docs.xen-orchestra.com/xoa#default-xo-account"
            >
              <FormInput
                v-model="xoaUser"
                :disabled="!isEditable"
                :required="isEditable"
                :placeholder="isEditable ? 'email@example.com' : 'admin@admin.net'"
              />
            </VtsInputWrapper>"#,
        },
        PatchDefinition {
            name: "Admin Password Fields Update",
            search: r#"          <div class="row">
            <VtsInputWrapper
              :label="t('admin-password')"
              learn-more-url="https://docs.xen-orchestra.com/xoa#default-xo-account"
            >
              <FormInput v-model="xoaPwd" type="password" required :placeholder="t('password')" />
            </VtsInputWrapper>
            <VtsInputWrapper
              :label="t('admin-password-confirm')"
              learn-more-url="https://docs.xen-orchestra.com/xoa#default-xo-account"
            >
              <FormInput v-model="xoaPwdConfirm" type="password" required :placeholder="t('password')" />
            </VtsInputWrapper>
          </div>"#,
            replace: r#"          <div class="row">
            <VtsInputWrapper
              :label="t('admin-password')"
              learn-more-url="https://docs.xen-orchestra.com/xoa#default-xo-account"
            >
              <FormInput
                v-model="xoaPwd"
                :type="isEditable ? 'password' : 'text'"
                :disabled="!isEditable"
                :required="isEditable"
                :placeholder="isEditable ? t('password') : 'admin'"
              />
            </VtsInputWrapper>
            <VtsInputWrapper
              :label="t('admin-password-confirm')"
              learn-more-url="https://docs.xen-orchestra.com/xoa#default-xo-account"
            >
              <FormInput
                v-model="xoaPwdConfirm"
                :type="isEditable ? 'password' : 'text'"
                :disabled="!isEditable"
                :required="isEditable"
                :placeholder="isEditable ? t('password') : 'admin'"
              />
            </VtsInputWrapper>
          </div>"#,
        },
        PatchDefinition {
            name: "SSH Account Login Properties",
            search: r#"        <FormSection :label="t('xoa-ssh-account')">
          <div class="row">
            <UiToggle v-model="enableSshAccount">{{ t('ssh-account') }}</UiToggle>
          </div>
          <div class="row">
            <VtsInputWrapper :label="t('ssh-login')">
              <FormInput value="xoa" placeholder="xoa" disabled />
            </VtsInputWrapper>
          </div>"#,
            replace: r#"        <FormSection :label="t('xoa-ssh-account')">
          <div class="row">
            <UiToggle v-model="enableSshAccount" :disabled="!isEditable">{{ t('ssh-account') }}</UiToggle>
          </div>
          <div class="row">
            <VtsInputWrapper :label="t('ssh-login')">
              <FormInput value="xo" placeholder="xo" disabled />
            </VtsInputWrapper>
          </div>"#,
        },
        PatchDefinition {
            name: "SSH Password Entry Configurations",
            search: r#"          <div class="row">
            <VtsInputWrapper :label="t('ssh-password')">
              <FormInput
                v-model="sshPwd"
                type="password"
                :placeholder="t('password')"
                :disabled="!enableSshAccount"
                :required="enableSshAccount"
              />
            </VtsInputWrapper>
            <VtsInputWrapper :label="t('ssh-password-confirm')">
              <FormInput
                v-model="sshPwdConfirm"
                type="password"
                :placeholder="t('password')"
                :disabled="!enableSshAccount"
                :required="enableSshAccount"
              />
            </VtsInputWrapper>
          </div>"#,
            replace: r#"          <div class="row">
            <VtsInputWrapper :label="t('ssh-password')">
              <FormInput
                v-model="sshPwd"
                :type="isEditable ? 'password' : 'text'"
                :placeholder="isEditable ? t('password') : 'xopass'"
                :disabled="!isEditable || !enableSshAccount"
                :required="isEditable && enableSshAccount"
              />
            </VtsInputWrapper>
            <VtsInputWrapper :label="t('ssh-password-confirm')">
              <FormInput
                v-model="sshPwdConfirm"
                :type="isEditable ? 'password' : 'text'"
                :placeholder="isEditable ? t('password') : 'xopass'"
                :disabled="!isEditable || !enableSshAccount"
                :required="isEditable && enableSshAccount"
              />
            </VtsInputWrapper>
          </div>"#,
        },
        PatchDefinition {
            name: "Proxy Core Constants Injection",
            search: "const REQUIRED_GB = 20",
            replace: r#"const REQUIRED_GB = 20

// ── XOA image sources ─────────────────────────────────────────────────────────
// Option 1 – XOA HomeLab: latest agent-built image, resolved at deploy time from
//            the Vagrantin/build-xoa-hl GitHub releases, routed through xoa-proxy.
// Option 2 – Vates official: VM.import called directly, no proxy.
// Option 3 – Ronivay community: routed through xoa-proxy (gzip + HTTP/HTTPS).
// Option 4 – Custom URL: same proxy path as ronivay.
const XOA_VATES_IMAGE_URL = 'http://xoa.io/xva'
const XOA_RONIVAY_IMAGE_URL = 'https://xo-image.yawn.fi/downloads/image.xva.gz'
const XOA_IMAGE_RELEASES_API =
  'https://api.github.com/repos/Vagrantin/build-xoa-hl/releases?per_page=30'
const XOA_HL_IMAGE_TAG_PREFIX = 'xoa-image-'

/**
 * Resolves the download URL of the newest agent-built XOA-HL image.
 * Images are published on build-xoa-hl, the repo they are built from; the
 * xoa-image- tag prefix and .xva/.xva.gz asset check are kept as a guard (the
 * same predicate the build agent uses). Sorted by published_at ourselves:
 * every xoa-image-* release built from the same build-xoa-hl commit shares
 * one created_at (the tag's target commit date), so trusting the API's
 * list order silently picks an arbitrary same-day release, not the newest.
 */
async function resolveXoaHlImageUrl(): Promise<string> {
  const response = await fetch(XOA_IMAGE_RELEASES_API)
  if (!response.ok) {
    throw new Error(`GitHub releases fetch failed: ${response.status}`)
  }
  const releases = (await response.json()) as {
    tag_name: string
    published_at: string
    assets: { name: string; browser_download_url: string }[]
  }[]
  const candidates = releases
    .filter(release => release.tag_name.startsWith(XOA_HL_IMAGE_TAG_PREFIX))
    .flatMap(release => {
      const asset = release.assets.find(a => a.name.endsWith('.xva') || a.name.endsWith('.xva.gz'))
      return asset === undefined ? [] : [{ release, asset }]
    })
    .sort((a, b) => Date.parse(b.release.published_at) - Date.parse(a.release.published_at))

  if (candidates.length === 0) {
    throw new Error('No XOA-HL image release found')
  }
  return candidates[0].asset.browser_download_url
}

type ImageSource = 'xoa-hl' | 'vates' | 'ronivay' | 'custom'

/** Which image source is currently selected. */
const imageSource = ref<ImageSource>('xoa-hl')

/**
 * User-entered URL, only used when imageSource === 'custom'.
 * The proxy auto-detects gzip vs raw XVA from the URL extension.
 */
const xoaImageUrl = ref('')

// ── Proxy config ──────────────────────────────────────────────────────────────
// XOA_PROXY_PORT must match the port xoa-proxy is listening on (default 9001).
const XOA_PROXY_PORT = 9001
const XOA_PROXY_BIND_ADDRESS = '127.0.0.1'

/**
 * When true (default), xoa-proxy verifies the upstream TLS certificate.
 * Uncheck only for self-signed / private-CA image servers.
 * Not applicable for the Vates path (no proxy used).
 */
const verifySsl = ref(true)

/**
 * Builds the local proxy URL that VM.import will pull from.
 *
 * Handles all four combinations transparently:
 * http  + .xva     → relay HTTP → HTTP, stream raw
 * http  + .xva.gz  → relay HTTP → HTTP, decompress gzip
 * https + .xva     → relay HTTPS → HTTP, stream raw
 * https + .xva.gz  → relay HTTPS → HTTP, decompress gzip
 */
function buildProxyUrl(sourceUrl: string, sslVerify: boolean): string {
  const params = new URLSearchParams({ src: sourceUrl })
  if (!sslVerify) {
    params.set('verify_ssl', 'false')
  }
  return `http://${XOA_PROXY_BIND_ADDRESS}:${XOA_PROXY_PORT}/image.xva?${params.toString()}`
}"#,
        },
        PatchDefinition {
            name: "Reactive State Credentials & Switches Hook",
            search: r#"const xoaUser = ref('')
const xoaPwd = ref('')
const xoaPwdConfirm = ref('')
const enableSshAccount = ref(true)
const sshPwd = ref('')
const sshPwdConfirm = ref('')"#,
            replace: r#"// Credential refs, values depend on imageSource (see watch below).
// Start empty because the default imageSource is 'xoa-hl', whose image applies
// the credentials entered here (via XenStore) at first boot.
const xoaUser = ref('')
const xoaPwd = ref('')
const xoaPwdConfirm = ref('')
const enableSshAccount = ref(true)
const sshPwd = ref('')
const sshPwdConfirm = ref('')

/**
 * True when fields should be editable (XOA HomeLab, Vates or Custom).
 * Only the Ronivay option uses baked-in, pre-filled credentials.
 */
const isEditable = computed(() => imageSource.value !== 'ronivay')

/**
 * When switching to the Ronivay path, show its baked-in defaults read-only.
 * When switching away, clear the fields so the user must fill in real
 * credentials (upstream behaviour).
 */
watch(imageSource, source => {
  if (source === 'ronivay') {
    xoaUser.value = 'admin@admin.net'
    xoaPwd.value = 'admin'
    xoaPwdConfirm.value = 'admin'
    sshPwd.value = 'xopass'
    sshPwdConfirm.value = 'xopass'
    enableSshAccount.value = true
  } else {
    xoaUser.value = ''
    xoaPwd.value = ''
    xoaPwdConfirm.value = ''
    sshPwd.value = ''
    sshPwdConfirm.value = ''
    enableSshAccount.value = true
  }
})"#,
        },
        PatchDefinition {
            name: "Log Message Exception Handling 1",
            search: r#"  if (xoaUser.value === '' || xoaPwd.value === '') {
    // Should not happen
    console.error('Missing XOA credentials')
    return
  }"#,
            replace: r#"  if (xoaUser.value === '' || xoaPwd.value === '') {
    // Should not happen
    console.error('Missing XOA user information')
    return
  }"#,
        },
        PatchDefinition {
            name: "Log Message Exception Handling 2",
            search: r#"  if (enableSshAccount.value && sshPwd.value === '') {
    // Should not happen
    console.error('Missing XOA credentials')
    return
  }"#,
            replace: r#"  if (enableSshAccount.value && sshPwd.value === '') {
    // Should not happen
    console.error('Missing XOA ssh credentials')
    return
  }"#,
        },
        PatchDefinition {
            name: "VM Import Proxy Redirection Routing Logic",
            search: r#"    vmRef.value = (
      (await xapi.call('VM.import', [
        'http://xoa.io/xva',
        selectedSr.value.$ref,
        false, // full_restore
        false, // force
      ])) as string[]
    )[0]"#,
            replace: r#"    // Vates: call VM.import directly, upstream behaviour, no proxy.
    // XOA HomeLab / Ronivay / Custom: route through xoa-proxy which handles
    // gzip decompression and both HTTP/HTTPS sources (including self-signed TLS).
    // The XOA HomeLab URL is resolved at deploy time from the latest GitHub release.
    let importUrl: string
    if (imageSource.value === 'vates') {
      importUrl = XOA_VATES_IMAGE_URL
    } else {
      const sourceUrl =
        imageSource.value === 'xoa-hl'
          ? await resolveXoaHlImageUrl()
          : imageSource.value === 'ronivay'
            ? XOA_RONIVAY_IMAGE_URL
            : xoaImageUrl.value
      importUrl = buildProxyUrl(sourceUrl, verifySsl.value)
    }

    vmRef.value = (
      (await xapi.call('VM.import', [
        importUrl,
        selectedSr.value.$ref,
        false, // full_restore
        false, // force
      ])) as string[]
    )[0]"#,
        },
        PatchDefinition {
            name: "VIF Deconstruction Conditional Wrap Guard",
            search: r#"    const [vifRef] = (await xapi.call('VM.get_VIFs', [vmRef.value])) as string[]
    await xapi.call('VIF.destroy', [vifRef])"#,
            replace: r#"    const [vifRef] = (await xapi.call('VM.get_VIFs', [vmRef.value])) as string[]

    if (vifRef !== undefined) {
      await xapi.call('VIF.destroy', [vifRef])
    }"#,
        },
        PatchDefinition {
            name: "CSS Scoped Layout Additions Append",
            search: "</style>",
            replace: r#".image-source-row {
  display: flex;
  gap: 1rem;
  align-items: center;
  width: 100%;
}

.image-source-select {
  height: 3.6rem;
  padding: 0 1rem;
  border: 1px solid var(--color-neutral-border-strong);
  border-radius: 0.4rem;
  background: var(--color-neutral-background-primary);
  color: var(--color-neutral-txt-primary);
  font-size: inherit;
  cursor: pointer;
  flex-shrink: 0;
}

.custom-url-input {
  flex: 1;
}

</style>"#,
        },
    ];

    // Pipeline execution stage with multi-occurrence assertions
    for patch in patches {
        let matches = content.matches(patch.search).count();
        if matches == 0 {
            eprintln!(
                "[PATCH FAILURE] Missing structural milestone anchor: '{}'",
                patch.name
            );
            process::exit(1);
        } else if matches > 1 {
            eprintln!(
                "[PATCH FAILURE] Ambiguous structural target. Landmark found {} times for: '{}'",
                matches, patch.name
            );
            process::exit(1);
        }

        content = content.replace(patch.search, patch.replace);
        println!("[OK] Applied milestone transformation: {}", patch.name);
    }

    if let Err(e) = fs::write(file_path, content) {
        eprintln!("Error: Failed to write patched data back to disk: {}", e);
        process::exit(1);
    }

    println!("[DONE] File structurally modernized successfully!");
}
