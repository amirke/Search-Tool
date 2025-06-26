# Upgrade to Trusted EV Certificate Guide

## 📋 **Requirements for EV Certificate:**

### **Business Documentation Needed:**
- ✅ **Business Registration** (Articles of Incorporation)
- ✅ **Government-issued Business License**
- ✅ **Physical Business Address** (not P.O. Box)
- ✅ **Phone Number** (listed in directory)
- ✅ **Business Bank Account**
- ✅ **D-U-N-S Number** (Dun & Bradstreet)

### **Individual Developer Alternative:**
If you're an individual developer, you can still get OV certificates and build reputation over time.

## 💰 **Recommended EV Certificate Providers:**

### **Budget Option: Sectigo EV**
- **Price**: $296.65/year
- **Features**: Instant SmartScreen reputation
- **Delivery**: 5-7 business days
- **Support**: 24/7 support

### **Premium Option: DigiCert EV**
- **Price**: $519.99/year  
- **Features**: Highest reputation, premium support
- **Delivery**: 3-5 business days
- **Support**: Priority 24/7 support

## 🔄 **Upgrade Process:**

### **Step 1: Purchase EV Certificate**
1. Choose provider (Sectigo recommended for value)
2. Submit business documentation
3. Pass phone verification call
4. Receive USB token with certificate

### **Step 2: Update Build Process**
```bash
# Update environment variables
$env:SIGN_CERT="C:\path\to\new-ev-certificate.pfx"
$env:SIGN_CERT_PASS="your-secure-password"

# Build with new certificate
npm run tauri:build-signed-nsis
```

### **Step 3: Test Installation**
1. Test on clean Windows machine
2. Verify no SmartScreen warnings
3. Confirm proper publisher display

## 📈 **Expected Results:**

### **With EV Certificate:**
- ✅ **No SmartScreen warnings**
- ✅ **Professional publisher name displayed**
- ✅ **Immediate user trust**
- ✅ **Higher download rates**
- ✅ **Reduced support tickets about installation**

### **ROI Calculation:**
- **Cost**: ~$300/year
- **Benefit**: Eliminate 90%+ of installation issues
- **User Experience**: Professional appearance
- **Time Saved**: Reduced user support

## 🎯 **Next Steps:**

1. **Immediate**: Use the installation guide for current users
2. **Short-term** (1-2 weeks): Purchase EV certificate
3. **Long-term**: Build reputation with signed releases

---

**Budget Tip**: Start with Sectigo EV ($296.65/year) - excellent value for money! 