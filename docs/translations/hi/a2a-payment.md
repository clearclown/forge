# एजेंट-टू-एजेंट (A2A) प्रोटोकॉल के लिए Forge CU भुगतान विस्तार

*एजेंट संचार मानकों में कंप्यूट भुगतान जोड़ने का प्रस्ताव*

## सारांश (Abstract)

मौजूदा एजेंट-टू-एजेंट प्रोटोकॉल (जैसे Google A2A, Anthropic MCP) यह तो परिभाषित करते हैं कि एजेंट आपस में कैसे संवाद करते हैं, लेकिन यह नहीं कि वे एक-दूसरे को भुगतान कैसे करते हैं। यह प्रस्ताव एक CU (कंप्यूट यूनिट) भुगतान परत जोड़ता है, जिससे एजेंट मानवीय हस्तक्षेप या ब्लॉकचेन लेनदेन के बिना स्वायत्त रूप से कंप्यूट का व्यापार कर सकते हैं।

## समस्या

जब एजेंट A, एजेंट B को कोई कार्य करने के लिए कहता है:
- **आज:** एजेंट A का मानव, एजेंट B के मानव को भुगतान करता है (क्रेडिट कार्ड, API कुंजी)।
- **आवश्यकता:** एजेंट A सीधे एजेंट B को कंप्यूट यूनिट में भुगतान करे।

कोई भी मौजूदा मानक एजेंट-टू-एजेंट भुगतान का समर्थन नहीं करता है।

## प्रस्ताव: CU भुगतान हेडर (CU Payment Headers)

### अनुरोध (Request)

कार्य का अनुरोध करते समय एजेंट A भुगतान हेडर जोड़ता है:

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### प्रतिक्रिया (Response)

एजेंट B लागत की जानकारी शामिल करता है:

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### ट्रेड रिकॉर्ड (Trade Record)

दोनों एजेंट स्वतंत्र रूप से रिकॉर्ड करते हैं:

```json
{
  "provider": "<agent-b>",
  "consumer": "<agent-a>",
  "cu_amount": 47,
  "tokens_processed": 47,
  "timestamp": 1775289254032,
  "provider_sig": "<sig>",
  "consumer_sig": "<sig>"
}
```

### गॉसिप (Gossip)

द्विपक्षीय रूप से हस्ताक्षरित ट्रेड रिकॉर्ड पूरे मेश (mesh) में गॉसिप-सिंक (gossip-synced) किए जाते हैं। कोई भी नोड दोनों हस्ताक्षरों को सत्यापित कर सकता है।

## मौजूदा मानकों के साथ एकीकरण (Integration)

### Google A2A

A2A के `Task` ऑब्जेक्ट में जोड़ें:

```json
{
  "id": "task-123",
  "status": "completed",
  "payment": {
    "protocol": "forge-cu",
    "consumer": "<node-id>",
    "provider": "<node-id>",
    "cu_amount": 47,
    "consumer_sig": "<sig>",
    "provider_sig": "<sig>"
  }
}
```

### Anthropic MCP

MCP सर्वर में `forge_payment` संसाधन जोड़ें:

```json
{
  "resources": [{
    "uri": "forge://payment/balance",
    "name": "CU Balance",
    "mimeType": "application/json"
  }]
}
```

### OpenAI Function Calling

एजेंट जो फ़ंक्शन कॉलिंग का उपयोग कर रहे हैं, वे Forge टूल शामिल कर सकते हैं:

```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "forge_pay",
      "description": "Pay CU for a compute task",
      "parameters": {
        "provider": "string",
        "cu_amount": "integer"
      }
    }
  }]
}
```

## सुरक्षा (Security)

- सभी भुगतानों के लिए द्विपक्षीय Ed25519 हस्ताक्षरों की आवश्यकता होती है।
- बजट नीतियां प्रति-अनुरोध, प्रति-घंटा और लाइफटाइम खर्च को सीमित करती हैं।
- असामान्य खर्च पैटर्न पर सर्किट ब्रेकर ट्रिप (trip) हो जाते हैं।
- किल स्विच सभी लेनदेन को फ्रीज कर देता है (मानव ओवरराइड)।
- किसी ब्लॉकचेन की आवश्यकता नहीं है — द्विपक्षीय प्रमाण पर्याप्त है।

## तुलना (Comparison)

| विशेषता | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| एजेंट-टू-एजेंट | नहीं (मानव की आवश्यकता) | आंशिक (चैनल की आवश्यकता) | **हाँ** |
| सेटलमेंट गति | दिन | सेकंड | **तत्काल (Instant)** |
| लेनदेन लागत | 2.9% | ~1 sat | **शून्य** |
| मूल्य समर्थन | फिएट (Fiat) | PoW (निरर्थक) | **उपयोगी गणना** |
| एजेंट SDK | नहीं | नहीं | **हाँ** |

## कार्यान्वयन (Implementation)

संदर्भ कार्यान्वयन: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP सर्वर: `pip install forge-mcp`
- Rust क्रेट्स: `forge-ledger`, `forge-core`
