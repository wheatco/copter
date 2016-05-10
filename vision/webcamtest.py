import cv2

cv2.namedWindow("preview")
vc = cv2.VideoCapture(0)

sift = cv2.xfeatures2d.SIFT_create()

if vc.isOpened():  # try to get the first frame
    rval, frame = vc.read()
else:
    rval = False

while rval:

    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

    kp = sift.detect(gray, None)
    img = cv2.drawKeypoints(gray, kp, frame, flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)

    cv2.imshow("preview", img)
    rval, frame = vc.read()
    cv2.imwrite('webcamtest.jpg', img)
    # key = cv2.waitKey(20)
    # if key == 27:  # exit on ESC
    #     break
cv2.destroyWindow("preview")
